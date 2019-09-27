/** @format */

export interface StyleMap {
  [property: string]: string;
}

/**
 * Render CSS style to KCSS style.
 * @param style A CSS styles map.
 */
export function renderStyle(style: StyleMap): StyleMap {
  return style;
}
