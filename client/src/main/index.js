/** @format */

import { app, BrowserWindow, ipcMain } from "electron";
import SystemFonts from "system-font-families";
import { format as formatUrl } from "url";
import * as path from "path";
import installExtension, { REDUX_DEVTOOLS } from "electron-devtools-installer";

const isDevelopment = process.env.NODE_ENV !== "production";
const systemFonts = new SystemFonts();

// Keep a global reference to the window object to prevent it from being
// destroyed by the garbage collector.
let mainWindow;

/**
 * Creates a new window.
 */
function createWindow() {
  mainWindow = new BrowserWindow({
    webPreferences: {
      nodeIntegration: true,
    },
  });

  if (isDevelopment) {
    mainWindow.webContents.openDevTools();
    mainWindow.loadURL(
      `http://localhost:${process.env.ELECTRON_WEBPACK_WDS_PORT}`,
    );
  } else {
    mainWindow.loadURL(
      formatUrl({
        pathname: path.join(__dirname, "index.html"),
        protocol: "file",
        slashes: true,
      }),
    );
  }

  mainWindow.on("closed", () => (mainWindow = null));
  mainWindow.webContents.on("devtools-opened", () => {
    mainWindow.focus();
    setImmediate(() => mainWindow.focus());
  });
}

/**
 * Installs custom dev tools.
 */
function installDevTools() {
  [REDUX_DEVTOOLS].forEach(extension => {
    installExtension(extension)
      .then(name => console.log(`Added Extension: ${name}`))
      .catch(err => console.log("An error occurred: ", err));
  });
}

app.on("ready", () => {
  installDevTools();
  createWindow();
});

app.on("window-all-closed", () => {
  // On Mac OS it is common for applications and their menu bar to stay active
  // until the user quits explicitly with Cmd + Q
  if (process.platform !== "darwin") app.quit();
});

app.on("activate", () => {
  // On Mac OS it is common to re-create a window in the app when the dock icon
  // is clicked and there are no other windows open.
  if (mainWindow === null) createWindow();
});

ipcMain.on("getFontList", (event, args) => {
  systemFonts.getFonts().then(
    res => {
      const fontArray = res
        .filter((font, index, arr) => arr.indexOf(font) === index)
        .sort();
      event.sender.send("fontList", fontArray);
    },
    err => console.log(err),
  );
});
