import { atom, read, update } from 'claude-code'
import type { Register, RenderElement, Timer } from 'claude-code'

// Seconds and minutes truncate; hours round to the half: 5s, 5m, 1h, 1.5h.
export const duration = (ms: number) => {
  const s = Math.floor(ms / 1000)
  if (s < 60) return `${s}s`
  const min = Math.floor(s / 60)
  if (min < 60) return `${min}m`
  return `${Math.round(min / 30) / 2}h`
}

// The live clock keeps its seconds so it visibly ticks: 5s, 1m 4s, 1h 2m 3s.
export const elapsed = (ms: number) => {
  const s = Math.floor(ms / 1000)
  const parts = [[Math.floor(s / 3600), 'h'], [Math.floor(s / 60) % 60, 'm'], [s % 60, 's']] as const
  const first = parts.findIndex(([n]) => n > 0)
  return parts.slice(first === -1 ? 2 : first).map(([n, unit]) => `${n}${unit}`).join(' ')
}

// Hint segments to drop: the interrupt and mode cycle hints, and the agent count (running agents are listed below the prompt anyway).
const NOISE = [/^esc to interrupt$/, /^\(shift\+tab to cycle\)$/, /^\/tasks to see subagents$/, /^← \d+ agents?$/]

export const quietHint = (hint: string, clock = '') =>
  [...hint.split(' · ').filter(part => !NOISE.some(re => re.test(part.trim()))), clock]
    .filter(Boolean)
    .join(' · ')

// In $.state, not a module variable: the mod reloads at turn end, after turn.complete has stored it.
const lastTurnMs = atom({ plugin: 'quiet-mode', key: 'lastTurnMs' } as const, null)

export const register: Register = on => {
  let elapsedMs: number | undefined
  let tick: Timer | undefined

  on('turn.start', async ($, e, next) => {
    if (!e.agentId) {
      tick?.cancel()
      const startedAt = await $.clock.now()
      elapsedMs = 0
      tick = $.clock.every(1000, async () => {
        elapsedMs = (await $.clock.now()) - startedAt
        $.ui.invalidate('ui.render')
      })
    }

    return next(e)
  })

  // Fires for aborted and errored turns too, so the clock always stops.
  on('turn.complete', async ($, e, next) => {
    if (!e.agentId) {
      tick?.cancel()
      tick = elapsedMs = undefined
      await update($, lastTurnMs, () => e.durationMs)
    }

    return next(e)
  })

  // The "⏵⏵ auto mode on · " pill and its separator are drawn outside this line, out of a mod's reach; rewriting `hint` keeps the engine's live pills.
  // The footer holds the clock: live while working (the spinner is gone), the last turn's duration after (the end-of-turn line is gone).
  on('ui.render', { component: 'PromptHint' }, async ($, e, next) => {
    const last = e.props.isWorking ? null : await read($, lastTurnMs)
    const clock = e.props.isWorking ? (elapsedMs === undefined ? '' : elapsed(elapsedMs)) : last === null ? '' : duration(last)
    const hint = quietHint(e.props.hint, clock)

    return hint === e.props.hint ? next(e) : next({ ...e, props: { ...e.props, hint } })
  })

  // An empty Box takes no rows, so the end-of-turn line is gone.
  on('ui.render', { component: 'TurnDuration' }, ($, e) => h($.ui.resolve(e).Box, {}) as RenderElement)

  // The engine draws the glyph and stats as one element, so the whole line goes. `message` (e.g. compacting) is real state, so keep it.
  on('ui.render', { component: 'Spinner' }, ($, e, next) =>
    e.props.message ? next(e) : (h($.ui.resolve(e).Box, {}) as RenderElement),
  )
}
