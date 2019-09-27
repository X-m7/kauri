/** @format */

/**
 * @TODO Calculate DPI dynamically
 */
const dpi: number = 96;

/**
 * A regular expression which can be used to split apart the numerical value and
 * unit string from a CSS value.
 *
 * ^           assert start of string
 * (           matching group
 *   \d*       zero or more numbers
 *   .?        an optional decimal point
 *   \d+       one or more numbers
 * )
 * ([a-zA-Z]+) matches one or more latin letters
 * $           assert end of string
 */
const UNIT_REGEX: RegExp = /^(\d*.?\d+)([a-zA-Z]+)$/;

/**
 * Converts a CSS value+unit into a number of pixels.
 * Note: this conversion relies heavily on the DPI of the user's screen.
 *
 * @param value Numerical value.
 * @param unit CSS unit.
 *
 * @example
 * convert(50, "mm")
 */
export const convert = (value: number, unit: string): number => {
  switch (unit) {
    case "mm":
      return ((value / 2.54) * dpi) / 10;
    case "cm":
      return (value / 2.54) * dpi;
    default:
      throw `Unknown unit '${unit}'.`;
  }
};

/**
 * Convert a CSS length to pixels.
 * @param length A CSS length.
 */
export function convertToPixels(length: string): number {
  const [_, valueStr, unit] = length.toLowerCase().match(UNIT_REGEX);
  const value = parseFloat(valueStr);
  return convert(value, unit);
}
