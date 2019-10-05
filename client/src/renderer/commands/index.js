/** @format */

/**
 * Add a new command object to the store
 * @param  {string}   id       command ID
 *                             must be of the form "namespace[.namespace].name"
 * @param  {string}   name     command name
 * @param  {Function} callback function to be called when command is executed
 *
 * @example
 *     createCommand("Clipboard.Copy", "Copy", Clipboard.doCopy)
 *     will register:
 *     {
 *       id: "Clipboard.Copy"
 *       name: "Copy",
 *       callback: Clipboard.doCopy,
 *     }
 */
export default (id, name, callback) => ({id, name, callback});