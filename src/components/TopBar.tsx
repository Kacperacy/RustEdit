import FileTab from "./files/FileTab";

function TopBar() {
  const files = [
    { name: "index.html", active: true },
    { name: "index.js", active: false },
    { name: "styles.css", active: false },
  ];

  return (
    <div className="flex flex-row w-10      ">
      {files.map((file) => (
        <FileTab name={file.name} isActive={file.active} />
      ))}
    </div>
  );
}

export default TopBar;
