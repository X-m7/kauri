/** @format */

const KEBAB_REGEX = /[A-Z]{2,}(?=[A-Z][a-z]+|\b)|[A-Z]?[a-z]+|[A-Z]/g;

/**
 * Convert a camelCase string into a kebab-case string.
 * @param {string} str String to convert to kebab-case.
 * @return {string} Kebab-case string.
 */
export function kebabize(str: string): string {
  return str
    .match(KEBAB_REGEX)
    .map(group => group.toLowerCase())
    .join("-");
}
