
export function estFonce(r: number, g:number, b:number): boolean {
  const luminance = 0.299 * r + 0.587 * g + 0.114 * b;
  return luminance <= 127;
}