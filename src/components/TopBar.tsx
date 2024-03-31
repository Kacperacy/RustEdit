import { useState } from "react";
import FileTab from "./files/FileTab";

interface File {
  name: string;
  filePath: string;
  active: boolean;
}

function TopBar() {
  const [files, setFiles] = useState<File[]>([
    { name: "file1", filePath: "file1", active: true },
    { name: "file2", filePath: "file2", active: false },
    { name: "file3", filePath: "file3", active: false },
  ]);

  function closeFile(event: React.MouseEvent, filePath: string) {
    event.stopPropagation();
    setFiles((prevFiles) =>
      prevFiles.filter((file) => file.filePath !== filePath)
    );
    if (files.length > 1) {
      setFileActive(files[0].filePath);
    }
  }

  function setFileActive(filePath: string) {
    setFiles((prevFiles) =>
      prevFiles.map((file) => ({
        ...file,
        active: file.filePath === filePath,
      }))
    );
  }

  return (
    <div className="flex flex-row bg-slate-700">
      {files.map((file) => (
        <FileTab
          key={file.filePath}
          name={file.name}
          filePath={file.filePath}
          isActive={file.active}
          onClick={setFileActive}
          onClose={closeFile}
        />
      ))}
    </div>
  );
}

export default TopBar;
