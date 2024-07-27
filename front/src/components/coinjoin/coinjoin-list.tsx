import { RoomDto } from "@/dtos";
import { Box, HStack, Spinner, Switch, Text, VStack } from "@chakra-ui/react";
import { FC, useState } from "react";
import { CoinjoinCard } from "..";
import moment from "moment";

interface IListCoinjoinRoom {
  isLoading: boolean;
  isError: boolean;
  deriv: string;
  data: RoomDto[];
}
export const CoinjoinList: FC<IListCoinjoinRoom> = (props) => {
  const [hideEnded, setHideEnded] = useState<boolean>(true);
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
        <Text>Failed to fetch list of coinjoin rooms</Text>
      </VStack>
    );
  return (
    <VStack h="100%" w="100%">
      <Box>
        <HStack>
          <Switch
            size="sm"
            isChecked={hideEnded}
            onChange={(e) => {
              console.log("value", e);
              setHideEnded((value) => !value);
            }}
          />
          <Text>{hideEnded ? "Show all" : "Hide ended rooms"}</Text>
        </HStack>
      </Box>
      {data
        ?.filter(
          (val) =>
            !hideEnded || val.created_at + val.due1 + val.due2 > moment.now(),
        )
        .map((val, index) => (
          <CoinjoinCard key={index} data={val} deriv={deriv} />
        ))}
    </VStack>
  );
};
