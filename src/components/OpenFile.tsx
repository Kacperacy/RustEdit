import { useContext } from "react";
import FilesContext from "../FilesContext";

const OpenFile: React.FC = () => {
  const { files, setFiles } = useContext(FilesContext);

  const handleFileOpen = () => {
    const input = document.createElement("input");
    input.type = "file";
    input.onchange = (event) => {
      if (!event.target) return;
      const fileInput = event.target as HTMLInputElement;

      if (!fileInput.files) return;
      const file = fileInput.files[0];
      if (file) {
        setFiles((prevFiles) => [
          ...prevFiles,
          {
            name: file.name,
            filePath: file.name,
            active: true,
            content: "",
          },
        ]);
      }
    };
    input.click();
  };

  return (
    <div className="flex flex-col items-center justify-center h-full w-full gap-4">
      <div className="text-3xl font-bold text-slate-200">
        Welcome to Rust-Edit
      </div>
      <div className="text-xl text-slate-300">Open a file to start coding</div>
      <button
        className="bg-slate-300 hover:bg-slate-100 rounded-md p-4"
        onClick={handleFileOpen}
      >
        Open File
      </button>
    </div>
  );
};

export default OpenFile;
