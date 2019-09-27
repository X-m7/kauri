/** @format */

import { h } from "preact";

/**
 * Renders a basic error page.
 * @param {function} props.retry A function to call to try again.
 * @param {Object} props.exception An error object.
 */
export default function Error(props) {
  // Handle network errors appearing as TypeErrors
  if (
    props.exception instanceof TypeError &&
    props.exception.message.includes("NetworkError")
  ) {
    props.exception = {
      name: "Network Error",
      message:
        "Unable to connect to remote server. Please check your " +
        "internet connection and try again.",
    };
  }

  return (
    <div class="error wrapper wrapper--thin">
      <h1>
        <span class="eyebrow">Error</span>
        {props.exception.name}
      </h1>

      {props.exception.message.length > 240 ? (
        <textarea class="error__message error__message--long" readOnly>
          {props.exception.message}
        </textarea>
      ) : (
        <p class="error__message">{props.exception.message}</p>
      )}
    </div>
  );
}
