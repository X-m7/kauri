/** @format */

import {
  workingArea as workingAreaReducer,
  nodesById as nodeReducer,
} from "./cache";
import { CACHE_WORKING_AREA, CACHE_NODE } from "../actions/types";

describe("cache node reducer", () => {
  it("should add a new node", () => {
    const state = {};
    const action = {
      type: CACHE_NODE,
      payload: { id: 5, height: 20 },
    };

    const expected = {
      5: {
        id: 5,
        height: 20,
        didInvalidate: false,
      },
    };

    expect(nodeReducer(state, action)).toEqual(expected);
  });
});

describe("cache working area reducer", () => {
  it("should save width and height", () => {
    const state = {};
    const action = {
      type: CACHE_WORKING_AREA,
      payload: { width: 10, height: 10 },
    };
    const expected = { width: 10, height: 10 };

    expect(workingAreaReducer(state, action)).toMatchObject(expected);
  });

  it("should override width and height", () => {
    const state = { width: 4, height: 4 };
    const action = {
      type: CACHE_WORKING_AREA,
      payload: { width: 15, height: 15 },
    };

    const expected = { width: 15, height: 15 };
    expect(workingAreaReducer(state, action)).toMatchObject(expected);
  });
});
