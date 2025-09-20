// @nrv/ui-kit — optional, built on @nrv/core applets (ADR-013)
// For now, export a simple story logger stub.

export type Logger = {
  section: (label: string) => Logger;
  log: (msg: string) => void;
  ok: (msg?: string) => void;
  fail: (msg?: string) => void;
};

export function logger(): Logger {
  return {
    section: (_label: string) => logger(),
    log: (_msg: string) => {},
    ok: (_msg?: string) => {},
    fail: (_msg?: string) => {},
  };
}
