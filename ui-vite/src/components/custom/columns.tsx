import * as React from "react";
import { CaretSortIcon, DotsHorizontalIcon } from "@radix-ui/react-icons";
import { ColumnDef } from "@tanstack/react-table";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import {
    DropdownMenu, DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger
} from "@/components/ui/dropdown-menu";
import { ArrowDownIcon, ArrowRightIcon, ArrowUpIcon, CircleIcon, MoreHorizontal } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { styleTokens } from "@/lib/constants";
import { getBaseUrl } from "@/lib/utils";
import { table } from "console";


export const data2: Survey[] =
    [{
        "blocks": [
            { "block_type": "Title", "id": "hf4li5b1fy8t", "index": 0.0, "properties": { "title": "User Registration Form", "type": "Title" } }, { "block_type": "TextInput", "id": "nnkucp2i699b", "index": 0.0, "properties": { "default": "John Dog", "question": "First name", "type": "TextInput" } }, { "block_type": "Empty", "id": "wupafcsj2yjy", "index": 0.0, "properties": { "type": "Nothing" } }],
        "created_at": "2024-01-14T00:40:29.971459Z",
        "id": 2, "modified_at": "2024-01-14T00:40:29.971474Z", "name": "name - todo", "parse_version": "2", "plaintext": "# User Registration Form\n\nText: First name [John Dog]", "survey_id": "n7wf1crdy3kh", "user_id": "usr_default2", "version": "version - todo", "workspace_id": "ws_default"
    }, {
        "blocks":
            [{ "block_type": "Title", "id": "z8uk113ayprw", "index": 0.0, "properties": { "title": "User Registration Form", "type": "Title" } }, { "block_type": "TextInput", "id": "yj1alcizau7j", "index": 0.0, "properties": { "default": "John Dog", "question": "First name", "type": "TextInput" } }, { "block_type": "TextInput", "id": "hxy5ehspwefi", "index": 0.0, "properties": { "default": "john@dog.com", "question": "Email Address", "type": "TextInput" } }, { "block_type": "Textarea", "id": "7qznmc661p6a", "index": 0.0, "properties": { "default": "Enter your comments here", "question": "This is nice", "type": "Textarea" } }, { "block_type": "Checkbox", "id": "1386c4b6vaam", "index": 0.0, "properties": { "options": [{ "checked": true, "id": "85d74zqlcunz", "text": "Subscribe to newsletter" }, { "checked": false, "id": "ul23qbi1laq9", "text": "second value here" }], "question": "subscribe?", "type": "Checkbox" } }, { "block_type": "Radio", "id": "nauxsg2padlp", "index": 0.0, "properties": { "options": ["radio button", "another one", "third radio"], "question": "my radio", "type": "Radio" } }, { "block_type": "Dropdown", "id": "apu67mvnly7u", "index": 0.0, "properties": { "options": ["Option 1", "Option 2", "Option 3"], "question": "My question here", "type": "Dropdown" } }, { "block_type": "Submit", "id": "atkl7vuhqdyw", "index": 0.0, "properties": { "default": "", "question": "submit", "type": "Submit" } }, { "block_type": "Empty", "id": "63p1uttvg5ey", "index": 0.0, "properties": { "type": "Nothing" } }], "created_at": "2024-01-16T04:57:27.448004Z", "id": 3, "modified_at": "2024-01-16T04:57:27.448015Z", "name": "name - todo", "parse_version": "2", "plaintext": "# User Registration Form\n\nText: First name [John Dog]\n\nText: Email Address [john@dog.com]\n\nTextarea: This is nice [Enter your comments here]\n\ncheckbox: subscribe?\n- [x] Subscribe to newsletter\n- [ ] second value here\n\nradio: my radio\n- radio button\n- another one\n- third radio\n\nDropdown: My question here\n  - Option 1\n  - Option 2\n  - Option 3\n\nSubmit: submit", "survey_id": "uam8mduu4cke", "user_id": "usr_default2", "version": "version - todo", "workspace_id": "ws_default"
    }].map((survey) => {
        return {
            id: survey.id,
            survey_id: survey.survey_id,
            version: survey.version,
            created_at: survey.created_at,
            plaintext: survey.plaintext,
            modified_at: survey.modified_at,
        }
    });

// export type Payment = {
//     id: string;
//     amount: number;
//     status: "pending" | "processing" | "success" | "failed";
//     email: string;
// };

export type Survey = {
    id: number;
    survey_id: string;
    version: string;
    created_at: string;
    // blocks: any[];
    modified_at: string;
    // name: string;
    // parse_version: string;
    plaintext: string;
    // user_id: string;
    // workspace_id: string;
};

export type FilterConfig = {
    name: string;
    displayName: string,
    type: string;
}

export type ColumnSettings = {
    name: string;
    displayName: string;
    sortable: boolean;
}

export type TableSettings = {
    columns: ColumnSettings[];
    filters: FilterConfig;

}

interface DataTableProps<TData, TValue> {
    columns: ColumnDef<TData, TValue>[]
    data: TData[]
}

function createColumnDef(columnDetail: ColumnSettings): ColumnDef<any> {

    return ({
        accessorKey: columnDetail.name,
        header: ({ column }) => {
            let sortable = columnDetail.sortable ? (
                < Button
                    variant="ghost"
                    onClick={() => column.toggleSorting(column.getIsSorted() === "asc")
                    }
                >
                    {columnDetail.displayName}
                    < CaretSortIcon className="ml-2 h-4 w-4" />
                </Button >)
                : <div className="text-right">{columnDetail.displayName}</div>
            return (
                sortable
            );
        },
        cell: ({ row }) => <div className="lowercase">{row.getValue(columnDetail.name)}</div>,
    })
}


export const surveyColumns: ColumnDef<any>[] = [
    {
        id: "select",
        header: ({ table }) => (
            <Checkbox
                checked={table.getIsAllPageRowsSelected() ||
                    (table.getIsSomePageRowsSelected() && "indeterminate")}
                onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
                aria-label="Select all" />
        ),
        cell: ({ row }) => (
            <Checkbox
                checked={row.getIsSelected()}
                onCheckedChange={(value) => row.toggleSelected(!!value)}
                aria-label="Select row" />
        ),
        enableSorting: false,
        enableHiding: false,
    },
    {
        accessorKey: "status",
        header: "Status",
        cell: ({ row }) => (
            <div className="capitalize">{row.getValue("status")}</div>
        ),
    },
    ...[
        { name: "survey_id", displayName: "ID", sortable: false },
        { name: "created_at", displayName: "Created", sortable: true },
        { name: "modified_at", displayName: "Modified", sortable: true },
        { name: "plaintext", displayName: "Plaintext", sortable: true },
    ].map(createColumnDef),
    {
        id: "actions",
        enableHiding: false,
        header: ({ column }) => <div className="text-right">{'Actions'}</div>,
        cell: ({ row }) => {
            const survey = row.original;
            const navigate = useNavigate();

            return (
                <DropdownMenu>
                    <DropdownMenuTrigger asChild className=''>
                        <Button variant="ghost" className="h-8 w-8 p-0">
                            <span className="sr-only">Open menu</span>
                            <DotsHorizontalIcon className="h-4 w-4" />
                        </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end" className="" style={{ backgroundColor: styleTokens.pink }}>
                        <DropdownMenuLabel>Actions</DropdownMenuLabel>
                        <DropdownMenuItem
                            onClick={(evt) => {
                                navigate(`/surveys/${survey.survey_id}`);
                            }}
                            className='hover:bg-blue-900'
                        >
                            View Survey
                        </DropdownMenuItem>
                        {/* <DropdownMenuSeparator /> */}
                        <DropdownMenuItem
                            onClick={(evt) => navigate(`/responses?survey_id=${survey.survey_id}`)}
                        >
                            Responses
                        </DropdownMenuItem>
                        <DropdownMenuItem
                            onClick={(evt) => navigator.clipboard.writeText(`${getBaseUrl()}/${survey.survey_id}`)}
                        >
                            Copy public link
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            );
        },
    },
];


const exampleResponsesData = {
    "responses": [
        {
            "answers": {
                "7qznmc661p6a": "a test",
                "85d74zqlcunz": "on",
                "hxy5ehspwefi": "is ",
                "nauxsg2padlp": "radio button",
                "ul23qbi1laq9": "on",
                "yj1alcizau7j": "this "
            },
            "id": 6,
            "submitted_at": "2024-01-16T04:57:50.080803Z",
            "survey_id": "uam8mduu4cke",
            "workspace_id": "ws_default"
        },
        {
            "answers": {
                "7qznmc661p6a": "",
                "85d74zqlcunz": "on",
                "hxy5ehspwefi": "",
                "yj1alcizau7j": ""
            },
            "id": 7,
            "submitted_at": "2024-01-21T07:19:02.826826Z",
            "survey_id": "uam8mduu4cke",
            "workspace_id": "ws_default"
        }
    ]
};

export type Answer = {
    id: string;
    questionText: string;
    value: string;
};

export type Response = {
    // answers: { [key: string]: string };
    answers: Answer[];
    id: number;
    submitted_at: string,
    survey_id: string;
    workspace_id: string;
};

export const responseData: Response[] = exampleResponsesData.responses.map(responses => {
    return {
        answers: [],
        id: responses.id,
        submitted_at: responses.submitted_at,
        survey_id: responses.survey_id,
        workspace_id: responses.workspace_id,
    }
});


export const responseColumns: ColumnDef<any>[] = [
    {
        id: "select",
        header: ({ table }) => (
            <Checkbox
                checked={table.getIsAllPageRowsSelected() ||
                    (table.getIsSomePageRowsSelected() && "indeterminate")}
                onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
                aria-label="Select all" />
        ),
        cell: ({ row }) => (
            <Checkbox
                checked={row.getIsSelected()}
                onCheckedChange={(value) => row.toggleSelected(!!value)}
                aria-label="Select row" />
        ),
        enableSorting: false,
        enableHiding: false,
    },
    {
        accessorKey: "status",
        header: "Status",
        cell: ({ row }) => (
            <div className="capitalize">{row.getValue("status")}</div>
        ),
    },
    ...[
        { name: "survey_id", displayName: "ID", sortable: false },
        { name: "created_at", displayName: "Created", sortable: true },
        { name: "modified_at", displayName: "Modified", sortable: true },
        { name: "plaintext", displayName: "Plaintext", sortable: true },
    ].map(createColumnDef),
    {
        id: "actions",
        enableHiding: false,
        header: ({ column }) => <div className="text-right">{'Actions'}</div>,
        cell: ({ row }) => {
            const survey = row.original;
            const navigate = useNavigate();

            return (
                <DropdownMenu>
                    <DropdownMenuTrigger asChild className=''>
                        <Button variant="ghost" className="h-8 w-8 p-0">
                            <span className="sr-only">Open menu</span>
                            <DotsHorizontalIcon className="h-4 w-4" />
                        </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end" className="" style={{ backgroundColor: styleTokens.pink }}>
                        <DropdownMenuLabel>Actions</DropdownMenuLabel>
                        <DropdownMenuItem
                            onClick={(evt) => {
                                navigate(`/surveys/${survey.survey_id}`);
                            }}
                            className='hover:bg-blue-900'
                        >
                            View Survey
                        </DropdownMenuItem>
                        {/* <DropdownMenuSeparator /> */}
                        <DropdownMenuItem
                            onClick={(evt) => navigate(`/responses?survey_id=${survey.survey_id}`)}
                        >
                            Responses
                        </DropdownMenuItem>
                        <DropdownMenuItem
                            onClick={(evt) => navigator.clipboard.writeText(`${getBaseUrl()}/${survey.survey_id}`)}
                        >
                            Copy public link
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            );
        },
    },
];