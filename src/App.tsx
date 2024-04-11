import { useState } from "react";
import FilesContext from "./FilesContext";
import FileTab from "./components/TopBar";
import OpenedFile from "./types/OpenedFile";
import Editor from "./components/Editor";
import OpenFile from "./components/OpenFile";

function App() {
  const [files, setFiles] = useState<OpenedFile[]>([]);

  return (
    <FilesContext.Provider value={{ files, setFiles }}>
      <div className="w-screen h-screen flex flex-col bg-slate-600 overflow-clip">
        {files.length > 0 ? <FileTab /> : null}
        {files.length > 0 ? <Editor /> : <OpenFile />}
      </div>
    </FilesContext.Provider>
  );
}

export default App;
