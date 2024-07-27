import { StateCoinTransferDto } from "@/dtos";
import {
  Spinner,
  Text,
  VStack,
  Button,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalBody,
  ModalFooter,
  useDisclosure,
} from "@chakra-ui/react";
import { FC, useState } from "react";
import { StateChainTransferCard } from "./statechain-transfer-card";

interface IStateChainTransferList {
  isLoading: boolean;
  isError: boolean;
  deriv: string;
  data: StateCoinTransferDto[];
}

export const StateChainTransferList: FC<IStateChainTransferList> = (props) => {
  const { deriv, data, isLoading, isError } = props;
  const { isOpen, onOpen, onClose } = useDisclosure();
  const [verifyError, setVerifyError] = useState<string>();

  if (isLoading)
    return (
      <VStack h="100%" w="100%">
        <Spinner />
      </VStack>
    );
  if (isError)
    return (
      <VStack h="100%" w="100%">
        <Text>Failed to fetch list of state chain</Text>
      </VStack>
    );
  return (
    <>
      <VStack h="100%" w="100%">
        {data?.map((val, index) => (
          <StateChainTransferCard data={val} key={index} deriv={deriv} />
        ))}
      </VStack>
      <Modal
        closeOnOverlayClick={false}
        isOpen={!!verifyError}
        onClose={onClose}
      >
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>ERROR !!!!</ModalHeader>

          <ModalBody pb={6}>{verifyError}</ModalBody>
          <ModalFooter>
            <Button
              colorScheme="red"
              onClick={() => {
                onClose();
                setVerifyError(undefined);
              }}
            >
              Cancel
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  );
};
