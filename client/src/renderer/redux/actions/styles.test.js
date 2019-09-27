/** @format */

import * as actions from "./styles";
import * as types from "./types";

describe("actions", () => {
  it("should create an action to set styles", () => {
    const key = "h1";
    const value = {
      display: "Heading 1",
      element: {
        type: "heading",
        level: 1,
      },
      styles: {
        color: "#333",
        fontsize: "2rem",
        spacing: "4rem 0 2rem",
      },
    };

    const expectedAction = {
      type: types.SET_STYLES,
      key,
      value,
    };
    expect(actions.setStyles(key, value)).toEqual(expectedAction);
  });
});
