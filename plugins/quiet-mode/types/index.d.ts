export type Ms = number | null

declare module 'claude-code' {
  interface PluginState {
    'quiet-mode': { lastTurnMs: Ms }
  }
}
