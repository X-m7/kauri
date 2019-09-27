/** @format */

import debugDocument from "./test.json";
import {
  MOVE_SELECTION,
  FETCH_DOCUMENT_REQUEST,
  FETCH_DOCUMENT_SUCCESS,
  FETCH_DOCUMENT_ERROR,
} from "../actions/types";
import { Status } from "../actions";
import { translateKDF } from "helpers/translateKDF";

const initialState = {
  status: Status.SUCCESS,
  selection: {
    start: 0,
    end: 0,
  },
  content: translateKDF(debugDocument),
};

/**
 * A reducer for document content.
 * @param {object} state Current state.
 * @param {object} action Action to perform.
 */
export default function documentReducer(state = initialState, action) {
  switch (action.type) {
    case FETCH_DOCUMENT_REQUEST:
      return {
        ...state,
        status: Status.LOADING,
      };

    case FETCH_DOCUMENT_SUCCESS:
      return {
        ...state,
        status: Status.SUCCESS,
        content: translateKDF(action.payload.content),
        lastUpdated: action.receivedAt,
      };

    case FETCH_DOCUMENT_ERROR:
      return {
        ...state,
        status: Status.ERROR,
        exception: action.exception,
      };

    case MOVE_SELECTION:
      return {
        ...state,
        selection: selectionReducer(state.selection, action),
      };

    default:
      return state;
  }
}

/**
 * A reducer for document selection.
 * @param {object} state Current state.
 * @param {object} action Action to perform on state.
 */
export function selectionReducer(state = { start: 0, end: 0 }, action) {
  switch (action.type) {
    case MOVE_SELECTION:
      return {
        ...state,
        start: action.start,
        end: action.end,
      };

    default:
      return state;
  }
}
