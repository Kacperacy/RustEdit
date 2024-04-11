interface FileTabProps {
  name: string;
  filePath: string;
  isActive: boolean;
  onClick: (filePath: string) => void;
  onClose: (event: React.MouseEvent, filePath: string) => void;
}

function FileTab(props: FileTabProps) {
  return (
    <div
      className={
        "border p-2 gap-2 border-slate-950 flex justify-between cursor-pointer select-none " +
        (props.isActive ? "bg-slate-600 border-t-orange-500 border-b-0" : "")
      }
      onClick={() => props.onClick(props.filePath)}
    >
      <div className="flex justify-center items-center">{props.name}</div>
      <div className="flex justify-center items-center">
        {props.isActive && (
          <svg
            xmlns="http://www.w3.org/2000/svg"
            x="0px"
            y="0px"
            viewBox="0 0 50 50"
            className="aspect-square w-4"
            onClick={(event) => props.onClose(event, props.filePath)}
          >
            <path d="M 9.15625 6.3125 L 6.3125 9.15625 L 22.15625 25 L 6.21875 40.96875 L 9.03125 43.78125 L 25 27.84375 L 40.9375 43.78125 L 43.78125 40.9375 L 27.84375 25 L 43.6875 9.15625 L 40.84375 6.3125 L 25 22.15625 Z"></path>
          </svg>
        )}
      </div>
    </div>
  );
}

export default FileTab;
