interface ICertificateUpload {
  isDisabled: boolean;
  isUploaded: boolean;
  setIsUploaded: (isUploaded: boolean) => void;
  communicatorHostname: string;
  className: string;
}

export default ICertificateUpload;
