import type { Register, RenderElement } from 'claude-code'

// Seconds and minutes truncate; hours round to the half: 5s, 5min, 1h, 1.5h.
export const duration = (ms: number) => {
  const s = Math.floor(ms / 1000)
  if (s < 60) return `${s}s`
  const min = Math.floor(s / 60)
  if (min < 60) return `${min}min`
  return `${Math.round(min / 30) / 2}h`
}

// Hint segments to drop: the mode cycle hint and the agent count (running agents are listed below the prompt anyway).
const NOISE = [/^\(shift\+tab to cycle\)$/, /^\/tasks to see subagents$/, /^← \d+ agents?$/]

export const quietHint = (hint: string, lastTurnMs?: number) =>
  [...hint.split(' · ').filter(part => !NOISE.some(re => re.test(part.trim()))), lastTurnMs === undefined ? '' : duration(lastTurnMs)]
    .filter(Boolean)
    .join(' · ')

export const register: Register = on => {
  let lastTurnMs: number | undefined

  on('turn.complete', ($, e, next) => {
    if (!e.agentId) {
      lastTurnMs = e.durationMs
      $.ui.invalidate('ui.render')
    }

    return next(e)
  })

  // The "⏵⏵ auto mode on · " pill and its separator are drawn outside this line, out of a mod's reach; rewriting `hint` keeps the engine's live pills.
  // The last turn's duration goes here, in place of the transcript's end-of-turn line; while working, the spinner has the clock.
  on('ui.render', { component: 'PromptHint' }, ($, e, next) => {
    const hint = quietHint(e.props.hint, e.props.isWorking ? undefined : lastTurnMs)

    return hint === e.props.hint ? next(e) : next({ ...e, props: { ...e.props, hint } })
  })

  // An empty Box takes no rows, so the end-of-turn line is gone.
  on('ui.render', { component: 'TurnDuration' }, ($, e) => h($.ui.resolve(e).Box, {}) as RenderElement)

  // Drop the whimsical verb; the icon and stats are the engine's and stay. `message` (e.g. compacting) is real state, so keep it.
  // An empty word is ignored (the engine draws its own), hence a space.
  on('ui.render', { component: 'Spinner' }, ($, e, next) =>
    e.props.message ? next(e) : next({ ...e, props: { ...e.props, word: ' ', suffix: '' } }),
  )
}
