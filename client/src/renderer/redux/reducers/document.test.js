/** @format */

import { Status } from "../actions";
import * as types from "../actions/types";
import documentReducer from "./document";

describe("document reducer", () => {
  it("should handle a FETCH_DOCUMENT_REQUEST action", () => {
    const state = { status: Status.NONE };
    const action = { type: types.FETCH_DOCUMENT_REQUEST };

    const expected = { status: Status.LOADING };

    expect(documentReducer(state, action)).toEqual(expected);
  });
});
