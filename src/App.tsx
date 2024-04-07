import { useState } from "react";
import FilesContext from "./FilesContext";
import FileTab from "./components/TopBar";
import OpenedFile from "./types/OpenedFile";
import Editor from "./components/Editor";

function App() {
  const [files, setFiles] = useState<OpenedFile[]>([
    {
      name: "index.html",
      filePath: "index.html",
      active: true,
    },
    {
      name: "styles.css",
      filePath: "styles.css",
      active: false,
    },
    {
      name: "script.js",
      filePath: "script.js",
      active: false,
    },
    {
      name: "README.md",
      filePath: "README.md",
      active: false,
    },
    {
      name: "package.json",
      filePath: "package.json",
      active: false,
    },
    {
      name: "tsconfig.json",
      filePath: "tsconfig.json",
      active: false,
    },
    {
      name: "webpack.config.js",
      filePath: "webpack.config.js",
      active: false,
    },
    {
      name: "babel.config.json",
      filePath: "babel.config.json",
      active: false,
    },
  ]);

  return (
    <FilesContext.Provider value={{ files, setFiles }}>
      <div className="w-screen h-screen bg-slate-600">
        <FileTab />
        <Editor />
      </div>
    </FilesContext.Provider>
  );
}

export default App;
