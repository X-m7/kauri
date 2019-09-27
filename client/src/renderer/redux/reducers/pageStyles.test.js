/** @format */

import pageStylesReducer from "./pageStyles";
import * as types from "../actions/types";

describe("page style reducer", () => {
  it("should handle SET_PAGE_STYLES", () => {
    expect(
      pageStylesReducer(
        { allIds: [], byId: {} },
        {
          type: types.SET_PAGE_STYLES,
          key: "page",
          value: {
            size: "A4",
            orientation: "portrait",
            margin: "2cm 3cm",
            borderWidth: "1px",
            borderStyle: "solid",
          },
        },
      ),
    ).toEqual({
      allIds: ["page"],
      byId: {
        page: {
          size: "A4",
          orientation: "portrait",
          margin: "2cm 3cm",
          borderWidth: "1px",
          borderStyle: "solid",
        },
      },
    });
  });
});
