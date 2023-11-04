interface IInterfaceForm {
  isEnabled: boolean;
  communicatorHostname: string;
  selectedGroup: string;
}

export const defaultFormData: IInterfaceForm = {
  isEnabled: false,
  communicatorHostname: "",
  selectedGroup: "",
};

export default IInterfaceForm;
