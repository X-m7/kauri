/** @format */

import { remote } from "electron";
import { fetchDocument } from "redux/actions";
import store from "redux/store";

const menuTemplate = [
  // Application menu for macOS
  ...(process.platform === "darwin"
    ? [
        {
          label: remote.app.getName(),
          submenu: [
            { role: "about" },
            { type: "separator" },
            { role: "services" },
            { type: "separator" },
            { role: "hide" },
            { role: "hideothers" },
            { role: "unhide" },
            { type: "separator" },
            { role: "quit" },
          ],
        },
      ]
    : []),
  // File menu
  {
    label: "File",
    submenu: [
      {
        label: "Open",
        click: () => {
          const path = remote.dialog.showOpenDialog({
            properties: ["openFile"],
          });
          store.dispatch(fetchDocument(path));
        },
      },
      process.platform === "darwin" ? { role: "close" } : { role: "quit" },
    ],
  },
  // Edit menu
  {
    label: "Edit",
    submenu: [
      { role: "undo" },
      { role: "redo" },
      { type: "separator" },
      { role: "cut" },
      { role: "copy" },
      { role: "paste" },
      ...(process.platform === "darwin"
        ? [
            { role: "pasteAndMatchStyle" },
            { role: "delete" },
            { role: "selectAll" },
            { type: "separator" },
            {
              label: "Speech",
              submenu: [{ role: "startspeaking" }, { role: "stopspeaking" }],
            },
          ]
        : [{ role: "delete" }, { type: "separator" }, { role: "selectAll" }]),
    ],
  },
  // View menu
  {
    label: "View",
    submenu: [
      { role: "reload" },
      { role: "forcereload" },
      { role: "toggledevtools" },
      { type: "separator" },
      { role: "resetzoom" },
      { role: "zoomin" },
      { role: "zoomout" },
      { type: "separator" },
      { role: "togglefullscreen" },
    ],
  },
  // Window menu
  {
    label: "Window",
    submenu: [
      { role: "minimize" },
      { role: "zoom" },
      ...(process.platform === "darwin"
        ? [
            { type: "separator" },
            { role: "front" },
            { type: "separator" },
            { role: "window" },
          ]
        : [{ role: "close" }]),
    ],
  },
];

export function configureMenu() {
  const menu = remote.Menu.buildFromTemplate(menuTemplate);
  remote.Menu.setApplicationMenu(menu);
}
