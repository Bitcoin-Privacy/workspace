import { StateCoinTransferDto } from "@/dtos";
import { Spinner, Text, VStack } from "@chakra-ui/react";
import { FC } from "react";
import { StateChainTransferCard } from "./statechain-transfer-card";

interface IStateChainTransferList {
  isLoading: boolean;
  isError: boolean;
  deriv: string;
  data: StateCoinTransferDto[];
}

export const StateChainTransferList: FC<IStateChainTransferList> = (props) => {
  const { deriv, data, isLoading, isError } = props;
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
    <VStack h="100%" w="100%">
      {data?.map((val, index) => (
        <StateChainTransferCard data={val} key={index} deriv={deriv} />
      ))}
    </VStack>
  );
};
