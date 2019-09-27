/** @format */

import { combineReducers } from "redux";
import { SET_PAGE_STYLES } from "../actions/types";

export default combineReducers({
  allIds: allStyles,
  byId: stylesById,
});

function allStyles(state = [], action) {
  switch (action.type) {
    case SET_PAGE_STYLES:
      return [...state, action.key];
    default:
      return state;
  }
}

function stylesById(state = {}, action) {
  switch (action.type) {
    case SET_PAGE_STYLES:
      return {
        [action.key]: action.value,
      };
    default:
      return state;
  }
}
