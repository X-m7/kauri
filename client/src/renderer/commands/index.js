/** @format */
import createShortcut from "./shortcuts.js";

/**
 * Construct a new command object
 * @param  {string}   id       command ID
 *                             must be of the form "namespace:name"
 * @param  {string}   name     command name
 * @param  {Function} callback function to be called when command is executed
 * @return {command}           command object
 *
 * @example
 *     createCommand("Clipboard:Copy", "Copy", Clipboard.doCopy)
 *     will return {
 *       shortcuts: [],
 *       id: "Clipboard:Copy"
 *       name: "Copy",
 *       callback: Clipboard.doCopy,
 *     }
 */
export function createCommand(id, name, callback) {
  return {
    id,
    name,
    shortcuts: [],
    callback,
  };
}

/**
 * Add a keyboard shortcut to an existing command
 * @param {command} command   registered command
 * @param {string} definition string description of keyboard shortcut
 *
 * @example
 *     addShortcut(Clipboard:Copy, "control+c")
 *     adds
 *     {
 *       "isAlt": false,
 *       "isCtrl": true,
 *       "isMeta": false,
 *       "isShift": false,
 *       "key": "c",
 *     }
 *     to the Clipboard:Copy command object's shortcuts array
 */
export function addShortcut(command, definition) {
  command.shortcuts.push(createShortcut(definition));
}
