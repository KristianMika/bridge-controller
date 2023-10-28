import { setCommunicatorCertificatePath } from "../../bindings";
import { open } from "@tauri-apps/api/dialog";
import { BsUpload } from "react-icons/bs";
import styles from "./CertificateUpload.module.css";
import React, { useState } from "react";
import * as path from "path";

interface ICertificationUpload {
  isDisabled: boolean;
  isUploaded: boolean;
  setIsUploaded: (isUploaded: boolean) => void;
  communicatorUrl: string;
  className: string;
}
export const CertificateUpload: React.FC<ICertificationUpload> = (props) => {
  const [filename, setFilename] = useState<string | null>(null);

  const uploadFile = (event: React.MouseEvent<HTMLElement>) => {
    event.preventDefault();
    open({
      multiple: false,
      directory: false,
      filters: [{ name: "PEM Certificates", extensions: ["pem"] }],
    }).then((filePath) => {
      if (filePath && typeof filePath === "string") {
        setFilename(filePath);
        setCommunicatorCertificatePath(filePath, props.communicatorUrl);
        props.setIsUploaded(true);
      }
    });
  };
  return (
    <button
      className={props.className}
      disabled={props.isDisabled}
      onClick={uploadFile}
      title={filename as string}
    >
      <div className={styles["certificate-button"]}>
        <span className={styles["certificate-button__filename"]}>
          {getButtonLabel(filename, props.isUploaded)}
        </span>
        <BsUpload size={21} />
      </div>
    </button>
  );
};

const getButtonLabel = (
  filename: string | null,
  isCertAlreadyUploaded: boolean
): string => {
  if (isCertAlreadyUploaded && !filename) {
    return "(stored)";
  }
  return getFilenameFromPath(filename);
};

const getFilenameFromPath = (filepath: string | null): string => {
  if (!filepath) {
    return "(none)";
  }

  let pathSeparator = path.sep || "/";

  let filename = filepath.split(pathSeparator).pop();
  if (!filename) {
    // invalid string, shouldn't happen as the open dialog filters *.pem files
    return "";
  }
  if (filename.length > 17) {
    filename = filename.slice(0, 15) + "...";
  }
  return filename;
};
