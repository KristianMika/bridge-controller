interface ICertificateUpload {
  isDisabled: boolean;
  isUploaded: boolean;
  setIsUploaded: (isUploaded: boolean) => void;
  communicatorUrl: string;
  className: string;
}

export default ICertificateUpload;
