/** @format */
import { genId } from "../helpers/uniqueIdGen.js";

/**
 * Construct new shortcut object
 * @param {object} definition keyboard shortcut object (made with parseShortcut)
 * @param {string} definition string describing keyboard shortcut, to be parsed
 * @return {object} keyboard shortcut object
 */
export default function createShortcut(definition) {
  if (typeof definition === "string") {
    definition = parseShortcut(definition);
  }

  return {
    id: genId(),
    isAlt: false,
    isCtrl: false,
    isMeta: false,
    isShift: false,
    ...definition,
  };
}

/**
 * Parse string into shortcut object
 * @param {string} str string describing shortcut
 *                        Must be of the form "modifier[+modifier]+key"
 *                        No spaces, key at end, modifiers in any order
 *                        Must not be empty
 * @return {object} keyboard shortcut object
 *
 * @example
 *     parseShortcut("control+c")
 *     will return {
 *       "isAlt": false,
 *       "isCtrl": true,
 *       "isMeta": false,
 *       "isShift": false,
 *       "key": "c",
 *     }
 */
export function parseShortcut(str) {
  if (str === "") throw "Cannot create an empty shortcut";

  const modifiers = str.toLowerCase().split("+");

  return {
    isAlt: modifiers.includes("alt"),
    isCtrl: modifiers.includes("control"),
    isMeta: modifiers.includes("meta"),
    isShift: modifiers.includes("shift"),
    key: modifiers[modifiers.length - 1],
  };
}

/**
 * Compare registered shortcut with keydown event
 * @param  {shortcut}       shortcut registered shortcut object
 * @param  {event} event    caught keydown event
 * @return {boolean}        true if keydown event matches shortcut description
 */
export function matchEvent(shortcut, event) {
  return (
    event.altKey === shortcut.isAlt &&
    event.ctrlKey === shortcut.isCtrl &&
    event.metaKey === shortcut.isMeta &&
    event.shiftKey === shortcut.isShift &&
    event.key === shortcut.key
  );
}
