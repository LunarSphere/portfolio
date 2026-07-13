export async function openExternal(url: string): Promise<void> {
  // With `noopener`, browsers may intentionally return null even when the new
  // tab opened successfully. Never use that return value to redirect this tab.
  window.open(url, "_blank", "noopener,noreferrer");
}
