import FilesContext from "./FilesContext";
import FileTab from "./components/TopBar";

function App() {
  const files = [
    { name: "file1", filePath: "file1", active: true },
    { name: "file2", filePath: "file2", active: false },
    { name: "file3", filePath: "file3", active: false },
  ];

  return (
    <FilesContext.Provider value={files}>
      <div className="w-screen h-screen bg-slate-600">
        <FileTab />
      </div>
    </FilesContext.Provider>
  );
}

export default App;
