import { expect, test } from 'claude-code/testing'
import type { RenderElement } from 'claude-code'
import { duration, quietHint } from './register'

test('drops the cycle hint and agent count from the hint line', () => {
  expect(quietHint('(shift+tab to cycle) · /tasks to see subagents · ← 1 agent')).toBe('')
  expect(quietHint('esc to interrupt · (shift+tab to cycle) · ← 3 agents')).toBe('esc to interrupt')
  expect(quietHint('? for shortcuts')).toBe('? for shortcuts')
})

test('hands the filtered hint on to the engine', async ($, on) => {
  let hint = ''
  on('ui.render', { component: 'PromptHint' }, ($, e) => {
    hint = e.props.hint
    return h($.ui.resolve(e).Text, {}, '') as RenderElement
  })

  await $.ui.render({
    surface: 'terminal',
    component: 'PromptHint',
    requestId: 'hint',
    props: { isDraft: false, isWorking: true, hint: 'esc to interrupt · (shift+tab to cycle)' },
  })

  expect(hint).toBe('esc to interrupt')
})

for (const [name, message, expected] of [
  ['blanks the spinner word', null, { word: ' ', suffix: '' }],
  ['keeps the word while a state message shows', 'Compacting', { word: 'Schlepping', suffix: '…' }],
] as const) {
  test(name, async ($, on) => {
    let seen = {}
    on('ui.render', { component: 'Spinner' }, ($, e) => {
      seen = { word: e.props.word, suffix: e.props.suffix }
      return h($.ui.resolve(e).Text, {}, '') as RenderElement
    })

    await $.ui.render({
      surface: 'terminal',
      component: 'Spinner',
      requestId: 'main',
      props: { word: 'Schlepping', message, suffix: '…', mode: 'responding' },
    })

    expect(seen).toEqual(expected)
  })
}

test('formats durations truncated, hours to the half', () => {
  expect([999, 5_400, 59_999, 60_000, 299_000, 3_600_000, 3_900_000, 4_500_000, 6_300_000].map(duration)).toEqual([
    '0s', '5s', '59s', '1min', '4min', '1h', '1h', '1.5h', '2h',
  ])
})

test('appends the last turn duration to the hint line', () => {
  expect(quietHint('(shift+tab to cycle)', 5_400)).toBe('5s')
  expect(quietHint('? for shortcuts', 300_000)).toBe('? for shortcuts · 5min')
})

test('draws nothing for the end-of-turn line', async $ => {
  const tree = await $.ui.render({
    surface: 'terminal',
    component: 'TurnDuration',
    requestId: 'turn-1',
    props: { word: 'Baked', durationMs: 5_400 },
  })

  expect(JSON.stringify(tree)).not.toMatch(/Baked|5s/)
})
