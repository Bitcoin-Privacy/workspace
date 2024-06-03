import { Box, Button, Text } from "@chakra-ui/react";

import { CoinJoinApi } from "@/apis";
import { FC } from "react";
import moment from "moment";
import TimeAgo from "timeago-react";

interface ICoinjoinStatus {
  deriv: string;
  roomId: string;
  endOfDue1: number;
  endOfDue2: number;
}

export const CoinjoinStatus: FC<ICoinjoinStatus> = (props) => {
  const { deriv, roomId, endOfDue1, endOfDue2 } = props;
  const now = moment().unix() * 1000;

  // TODO: Check status of the room
  // Signed?
  // Completed?
  if (now < endOfDue1)
    return (
      <Box textAlign="right">
        <Text fontSize="16px" fontWeight="700" w="100%">
          Waiting for other peers...
        </Text>
        <Text fontSize="14px" fontWeight="500" w="100%">
          {"Move to the next step "}
          <TimeAgo datetime={endOfDue1} />
        </Text>
      </Box>
    );
  else if (now < endOfDue2)
    return (
      <Box textAlign="right">
        <Button
          onClick={() => {
            CoinJoinApi.signTxn(deriv, roomId);
          }}
        >
          Sign
        </Button>
      </Box>
    );
  else
    return (
      <Box textAlign="right">
        <Text fontSize="16px" fontWeight="700" w="100%">
          Ended
        </Text>
      </Box>
    );
};
