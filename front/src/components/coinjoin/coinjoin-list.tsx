import { RoomDto } from "@/dtos";
import { Spinner, Text, VStack } from "@chakra-ui/react";
import { FC } from "react";
import { CoinjoinCard } from "..";

interface IListCoinjoinRoom {
  isLoading: boolean;
  isError: boolean;
  deriv: string;
  data: RoomDto[];
}
export const CoinjoinList: FC<IListCoinjoinRoom> = (props) => {
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
        <CoinjoinCard key={index} data={val} deriv={deriv} />
      ))}
    </VStack>
  );
};
