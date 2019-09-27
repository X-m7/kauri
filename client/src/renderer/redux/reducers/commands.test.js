/** @format */

import commandReducer from "./commands";
import * as types from "../actions/types";

describe("command reducer", () => {
  it("should handle ADD_COMMAND", () => {
    expect(
      commandReducer(
        { allIds: [], byId: {} },
        {
          type: types.ADD_COMMAND,
          command: {
            id: "clipboard:copy",
            name: "copy",
            shortcuts: [
              {
                id: 1,
                isAlt: false,
                isCtrl: false,
                isMeta: false,
                isShift: false,
              },
            ],
            callback: "this.doClipboardCopy",
          },
        },
      ),
    ).toEqual({
      allIds: ["clipboard:copy"],
      byId: {
        "clipboard:copy": {
          id: "clipboard:copy",
          name: "copy",
          shortcuts: [
            {
              id: 1,
              isAlt: false,
              isCtrl: false,
              isMeta: false,
              isShift: false,
            },
          ],
          callback: "this.doClipboardCopy",
        },
      },
    });
  });
});
