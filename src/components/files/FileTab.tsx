interface FileTabProps {
  name: string;
  isActive: boolean;
}

function FileTab(props: FileTabProps) {
  return (
    <div>
      <p>{props.name}</p>
    </div>
  );
}

export default FileTab;
