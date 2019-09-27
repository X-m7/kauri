/** @format */

import metadataReducer from "./metadata";
import * as types from "../actions/types";

describe("metadata reducer", () => {
  it("should handle SET_METADATA", () => {
    expect(
      metadataReducer(
        {
          title: "Let's Rethink Document Processing",
        },
        {
          type: types.SET_METADATA,
          key: "title",
          data: "Updated title",
        },
      ),
    ).toEqual({
      title: "Updated title",
    });
  });

  it("should add new metadata", () => {
    expect(
      metadataReducer(
        {
          title: "Let's Rethink Document Processing",
        },
        {
          type: types.SET_METADATA,
          key: "layout",
          data: "design",
        },
      ),
    ).toEqual({
      title: "Let's Rethink Document Processing",
      layout: "design",
    });
  });
});
