import {createApi, fetchBaseQuery} from '@reduxjs/toolkit/query/react';
import {RawData} from "../models/imudata";

export const apiSlice = createApi({
  reducerPath: 'api',
  baseQuery: fetchBaseQuery({
    baseUrl: '/',
  }),
  tagTypes: ['Deployment',
    'Exercise',
    'Score',
    'Scenario',
    'Participant',
    'ManualMetric',
    'Email'],
  endpoints: builder => ({
    getRawData: builder.query<RawData, string>({
      query: exerciseId => `/imudata`,
    }),
    addRawData: builder
      .mutation<string,
        {fileContent: string}>({
        query: ({fileContent}) => ({
          url: `/imudata`,
          method: 'POST',
          body: fileContent,
        }),
      }),
  }),
});

export const {
  useGetRawDataQuery,
  useAddRawDataMutation,
} = apiSlice;
