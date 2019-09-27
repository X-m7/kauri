/** @format */

import * as actions from "./document";
import * as types from "./types";

describe("fetch document request action creator", () => {
  it("should return the document request action", () => {
    const expectedAction = { type: types.FETCH_DOCUMENT_REQUEST };
    expect(actions.fetchDocumentRequest()).toEqual(expectedAction);
  });
});

describe("fetch document error action creator", () => {
  it("should return document error action and exception message", () => {
    const exception = "error";
    const expectedAction = { type: types.FETCH_DOCUMENT_ERROR, exception };
    expect(actions.fetchDocumentError(exception)).toEqual(expectedAction);
  });
});

describe("fetch document success action creator", () => {
  it("should return document fetch success action + payload", () => {
    jest
      .spyOn(global.Date, "now")
      .mockImplementationOnce(() => new Date("2019-09-21 13:28:00"));

    const payload = "json";
    const expectedAction = {
      type: types.FETCH_DOCUMENT_SUCCESS,
      payload,
      receivedAt: new Date("2019-09-21 13:28:00"),
    };

    expect(actions.fetchDocumentSuccess(payload)).toEqual(expectedAction);
  });
});

describe("move selection action creator", () => {
  it("should return an action to move the selection", () => {
    const start = 0;
    const end = 5;
    const expectedAction = {
      type: types.MOVE_SELECTION,
      start,
      end,
    };

    expect(actions.moveSelection(start, end)).toEqual(expectedAction);
  });
});
