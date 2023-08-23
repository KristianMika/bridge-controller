import { setCommunicatorCertificatePath } from "../../bindings";
import { open } from "@tauri-apps/api/dialog";

interface ICertificationUpload {
  isDisabled: boolean;
  communicatorUrl: string;
  className: string;
}
export const CertificateUpload: React.FC<ICertificationUpload> = (props) => {
  const uploadFile = (event: React.MouseEvent<HTMLElement>) => {
    event.preventDefault();
    open({
      multiple: false,
      directory: false,
      filters: [{ name: "PEM Certificates", extensions: ["pem"] }],
    }).then((filePath) => {
      if (filePath && typeof filePath === "string") {
        setCommunicatorCertificatePath(filePath, props.communicatorUrl);
      }
    });
  };
  return (
    <button
      className={props.className}
      disabled={props.isDisabled}
      onClick={uploadFile}
    >
      Upload
    </button>
  );
};
