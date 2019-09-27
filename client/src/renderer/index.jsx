/** @format */

import "styles/application.scss";

import { h, render } from "preact";
import { Provider } from "react-redux";

import Application from "components/Application";
import store from "redux/store";

render(
  <Provider store={store}>
    <Application />
  </Provider>,
  document.body,
);
