/** @format */

import {
  FETCH_DOCUMENT_ERROR,
  FETCH_DOCUMENT_REQUEST,
  FETCH_DOCUMENT_SUCCESS,
  MOVE_SELECTION,
} from "./types";
import { Status } from "./index";

/**
 * An action creator which handles a new fetch request.
 */
export const fetchDocumentRequest = () => ({
  type: FETCH_DOCUMENT_REQUEST,
});

/**
 * An action creator which handles a failed document fetch.
 * @param {object} exception Fetch exception.
 */
export const fetchDocumentError = exception => ({
  type: FETCH_DOCUMENT_ERROR,
  exception,
});

/**
 * An action creator which handles a successful document fetch.
 * @param {object} payload JSON payload from server.
 */
export const fetchDocumentSuccess = payload => ({
  type: FETCH_DOCUMENT_SUCCESS,
  payload,
  receivedAt: Date.now(),
});

/**
 * Fetch a parsed document from the file system.
 * @param {string} path Path to document.
 */
export function fetchDocument(path) {
  return dispatch => {
    dispatch(fetchDocumentRequest());

    fetch("http://localhost:3000/load", {
      method: "POST",
      body: path,
    })
      .then(response => response.json())
      .then(json => dispatch(fetchDocumentSuccess(json)))
      .catch(exception => dispatch(fetchDocumentError(exception)));
  };
}

/**
 * Whether a fetch action should be performed.
 * @param {object} state Current store state.
 */
export function shouldFetchDocument(state) {
  return state.document.status !== Status.LOADING;
}

/**
 * Fetch the document at the given path, but only if currently possible.
 * @param {string} path Path to document.
 */
export function fetchDocumentIfShould(path) {
  return (dispatch, getState) => {
    if (shouldFetchDocument(getState())) {
      return dispatch(fetchDocument(path));
    }

    return Promise.resolve();
  };
}

/**
 * Move the current document selection position.
 * @param {number} start Start position.
 * @param {number} end End position.
 */
export const moveSelection = (start, end) => ({
  type: MOVE_SELECTION,
  start,
  end,
});
