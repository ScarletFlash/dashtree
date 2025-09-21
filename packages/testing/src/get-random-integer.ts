export function getRandomInteger(min = 0, max = 100): number {
  const sanitizedMin = Math.round(min);
  const sanitizedMax = Math.round(max);

  if (sanitizedMin > sanitizedMax) {
    throw new RangeError(
      `[min: ${sanitizedMin}] must be less than or equal to [max: ${sanitizedMax}]`,
    );
  }

  const minSpan: number = sanitizedMin === -Infinity ? 0 : sanitizedMin;
  const maxSpan: number =
    sanitizedMax === Infinity ? 0 : sanitizedMax - minSpan;

  return Math.floor(Math.random() * (maxSpan + 1)) + minSpan;
}
