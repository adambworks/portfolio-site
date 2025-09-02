interface ConfirmationModalProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: () => void;
  message: string;
}

export default function ConfirmationModal({ isOpen, onClose, onConfirm, message }: ConfirmationModalProps) {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div className="rounded-lg bg-white p-6 shadow-lg">
        <p className="mb-4 text-lg">{message}</p>
        <div className="flex justify-end">
          <button onClick={onClose} className="mr-2 rounded bg-gray-300 px-4 py-2 text-black">
            Cancel
          </button>
          <button
            onClick={() => {
              onConfirm();
              onClose();
            }}
            className="rounded bg-red-500 px-4 py-2 text-white"
          >
            Confirm
          </button>
        </div>
      </div>
    </div>
  );
}