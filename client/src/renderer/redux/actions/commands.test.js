/** @format */

import { createCommand, addShortcut } from "../../commands";
import * as actions from "./commands";
import * as types from "./types";

describe("command actions", () => {
  it("should create an action to add a command", () => {
    const id = "clipboard:copy";
    const name = "copy";
    const keys = "control+c";
    const callback = "this.doClipboardCopy";

    const cmd = createCommand(id, name, callback);
    addShortcut(cmd, keys);

    const expectedAction = {
      type: types.ADD_COMMAND,
      command: {
        id,
        name,
        shortcuts: [
          {
            id: 1,
            isAlt: false,
            isCtrl: true,
            isMeta: false,
            isShift: false,
            key: "c",
          },
        ],
        callback,
      },
    };
    expect(actions.addCommand(cmd)).toEqual(expectedAction);
  });
});
