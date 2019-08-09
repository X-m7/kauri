/** @format */
import { combineReducers } from "redux";
import { ADD_COMMAND } from "../actionTypes";

const initialState = {
  commands: {
    allIds: [],
    byId: {},
  },
};

export default function rootReducer(state = initialState, action) {
  switch (action.type) {
    case ADD_COMMAND:
      return {
        ...state,
        commands: commandsReducer(state.commands, action),
      };
    default:
      return state;
  }
}

const commandsReducer = combineReducers({
  allIds: allCommands,
  byId: commandsById,
});

function allCommands(state = [], action) {
  switch (action.type) {
    case ADD_COMMAND:
      return [...state.allIds, action.payload.id];
    default:
      return state;
  }
}

function commandsById(state = {}, action) {
  switch (action.type) {
    case ADD_COMMAND:
      return {
        ...state,
        [action.payload.id]: { ...action.payload },
      };
    default:
      return state;
  }
}