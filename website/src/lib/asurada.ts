const ASURADA_URL = import.meta.env.VITE_ASURADA_URL ?? "http://localhost:7878";

export const signal = async (opts: {
  domain: string;
  event: string;
  project?: string;
  title?: string;
  payload?: Record<string, unknown>;
}): Promise<void> => {
  try {
    await fetch(`${ASURADA_URL}/signal`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(opts),
      signal: AbortSignal.timeout(2000),
    });
  } catch {}
};
