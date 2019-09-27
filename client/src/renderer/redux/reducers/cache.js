/** @format */

import { combineReducers } from "redux";
import {
  CACHE_WORKING_AREA,
  CACHE_NODE,
  FETCH_DOCUMENT_SUCCESS,
} from "../actions/types";

export default combineReducers({ nodesById, workingArea });

export function nodesById(state = {}, action) {
  switch (action.type) {
    case FETCH_DOCUMENT_SUCCESS:
      return {};

    case CACHE_NODE:
      return {
        ...state,
        [action.payload.id]: {
          ...action.payload,
          didInvalidate: false,
        },
      };

    default:
      return state;
  }
}

export function workingArea(state = { didInvalidate: true }, action) {
  switch (action.type) {
    case FETCH_DOCUMENT_SUCCESS:
      return {
        didInvalidate: true,
      };

    case CACHE_WORKING_AREA:
      return {
        ...action.payload,
        didInvalidate: false,
      };

    default:
      return state;
  }
}
