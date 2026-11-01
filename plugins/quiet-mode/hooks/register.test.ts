import { expect, mock, test } from 'claude-code/testing'
import type { RenderElement } from 'claude-code'
import { duration, elapsed, quietHint } from './register'

test('drops the interrupt and cycle hints and agent count from the hint line', () => {
  expect(quietHint('(shift+tab to cycle) · /tasks to see subagents · ← 1 agent')).toBe('')
  expect(quietHint('esc to interrupt · (shift+tab to cycle) · ← 3 agents')).toBe('')
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
    props: { isDraft: false, isWorking: true, hint: '? for shortcuts · (shift+tab to cycle)' },
  })

  expect(hint).toBe('? for shortcuts')
})

test('draws nothing for the spinner', async $ => {
  const tree = await $.ui.render({
    surface: 'terminal',
    component: 'Spinner',
    requestId: 'main',
    props: { word: 'Schlepping', message: null, suffix: '…', mode: 'responding' },
  })

  expect(JSON.stringify(tree)).not.toMatch(/Schlepping|…/)
})

test('keeps the spinner while a state message shows', async ($, on) => {
  let drawn = false
  on('ui.render', { component: 'Spinner' }, ($, e) => {
    drawn = true
    return h($.ui.resolve(e).Text, {}, '') as RenderElement
  })

  await $.ui.render({
    surface: 'terminal',
    component: 'Spinner',
    requestId: 'main',
    props: { word: 'Schlepping', message: 'Compacting', suffix: '…', mode: 'responding' },
  })

  expect(drawn).toBe(true)
})

test('the footer runs a clock while working, then shows the turn duration', async ($, on) => {
  const clock = mock.clock(on)
  on('turn.start', (_, e) => ({ turnId: e.turnId }))
  on('turn.complete', () => ({ text: '' }))
  let hint = ''
  on('ui.render', { component: 'PromptHint' }, ($, e) => {
    hint = e.props.hint
    return h($.ui.resolve(e).Text, {}, '') as RenderElement
  })
  const footer = (isWorking: boolean) =>
    $.ui.render({
      surface: 'terminal',
      component: 'PromptHint',
      requestId: 'hint',
      props: { isDraft: false, isWorking, hint: 'esc to interrupt' },
    })

  await $.turn.start({ text: 'hi', turnId: 't1' })
  await clock.advance(64_000)
  await footer(true)
  expect(hint).toBe('1m 4s')

  await $.turn.complete({ answer: '', durationMs: 5_400, isAborted: false, turnId: 't1', reason: 'answer' })
  await clock.advance(3_000)
  await footer(false)
  expect(hint).toBe('5s')
})

test('the last turn duration survives a reload', async ($, on) => {
  // What the copy before the reload stored.
  on('state.get', () => ({ value: { value: 5_400, version: 1 } }))
  let hint = ''
  on('ui.render', { component: 'PromptHint' }, ($, e) => {
    hint = e.props.hint
    return h($.ui.resolve(e).Text, {}, '') as RenderElement
  })

  await $.ui.render({
    surface: 'terminal',
    component: 'PromptHint',
    requestId: 'hint',
    props: { isDraft: false, isWorking: false, hint: '' },
  })

  expect(hint).toBe('5s')
})

test('formats durations truncated, hours to the half', () => {
  expect([999, 5_400, 59_999, 60_000, 299_000, 3_600_000, 3_900_000, 4_500_000, 6_300_000].map(duration)).toEqual([
    '0s', '5s', '59s', '1m', '4m', '1h', '1h', '1.5h', '2h',
  ])
})

test('formats the live clock with its seconds', () => {
  expect([999, 5_400, 64_000, 3_600_000, 3_723_000].map(elapsed)).toEqual(['0s', '5s', '1m 4s', '1h 0m 0s', '1h 2m 3s'])
})

test('appends the clock to the hint line', () => {
  expect(quietHint('(shift+tab to cycle)', '5s')).toBe('5s')
  expect(quietHint('? for shortcuts', '5m')).toBe('? for shortcuts · 5m')
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
