import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { useGetSurvey } from "../../hooks/useGetSurvey.ts";
import { createTable } from "./createTable.tsx";
import {
  getBaseUrl,
  getSessionToken,
  handleResponse,
} from "../../lib/utils.ts";
import { DataTable } from "./data-table.tsx";
import {
  surveyColumns,
  data2,
  mapRealQuestionToAnswers,
  mapAnswersToColumns,
  GetResponses,
} from "./columns.tsx";

export function ListResponses() {
  const [surveyResponses, setSurveyResponses] = useState<
    GetResponses | undefined
  >(undefined);
  const [queryParams, setQueryParams] = useSearchParams();
  // let globalState: GlobalState = useContext(GlobalStateContext);
  const SURVEY_ID_QUERY_KEY = "survey_id";
  // let { survey, error, isPending } = useGetSurvey(queryParams.get(SURVEY_ID_QUERY_KEY) || '');

  useEffect(() => {
    getResponses(queryParams.get(SURVEY_ID_QUERY_KEY) || "");
  }, [queryParams]);

  async function getResponses(survey_id: string) {
    if (!survey_id) {
      console.log("ListResponses - survey_id not defined");
      return;
    }

    const response = await fetch(
      `${getBaseUrl()}/v1/responses?${new URLSearchParams({
        survey_id: queryParams.get(SURVEY_ID_QUERY_KEY) || "",
      })}`,
      {
        method: "GET",
        headers: {
          session_id: getSessionToken(),
        },
      }
    );
    handleResponse(response);
    const result = await response.json();
    console.log("data: ", result);
    if (result.error) {
      console.log("failed to get surveys: ", result);
      // setError(result.message ?? 'Generic error getting surveys');
      if (response.status === 401) {
        // redirect({ to: "/login", replace: true });
      }
    } else {
      console.log("Found surveys: ", result);
      setSurveyResponses(result);
      // setError('');
    }
  }

  console.log("ListResponses - data: ", JSON.stringify(surveyResponses));
  let responseColumns = mapAnswersToColumns(surveyResponses);
  let tableData = mapRealQuestionToAnswers(surveyResponses);
  console.log("final data: ", tableData);

  return (
    <>
      <div>
        {/* <pre>
                    {JSON.stringify(tableData, null, 2)}
                </pre> */}
        {/* {createTable(columns, ['id', 'submitted_at', ...Object.keys(idToTitle).map((key) => 'answers.' + key)], surveyResponses)} */}
      </div>
      {surveyResponses ? (
        <div className="container py-10 mx-auto">
          {/* <DataTable columns={columns} data={data} /> */}
          <DataTable columns={responseColumns ?? []} data={tableData ?? []} />
        </div>
      ) : (
        <h2>Not available</h2>
      )}
    </>
  );
}
