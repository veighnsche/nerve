// @nrv/core — minimal stubs to compile; no hidden behavior (ADR-008)

export type Step = {
  info: (msg: string) => void;
  ok: (msg?: string) => void;
  fail: (msg?: string) => void;
};

export const ui = {
  step(label: string): Step {
    return {
      info: (_msg: string) => {},
      ok: (_msg?: string) => {},
      fail: (_msg?: string) => {},
    };
  },
};

export function version(): string {
  return "0.1.0";
}
