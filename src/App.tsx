import { useState } from "react";
import FilesContext from "./FilesContext";
import FileTab from "./components/TopBar";
import OpenedFile from "./types/OpenedFile";

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
  ]);

  return (
    <FilesContext.Provider value={{ files, setFiles }}>
      <div className="w-screen h-screen bg-slate-600">
        <FileTab />
      </div>
    </FilesContext.Provider>
  );
}

export default App;
